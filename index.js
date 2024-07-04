/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */

/* auto-generated by NAPI-RS */

const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'node.android-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./node.android-arm64.node')
          } else {
            nativeBinding = require('@integrationos/node-android-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'node.android-arm-eabi.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./node.android-arm-eabi.node')
          } else {
            nativeBinding = require('@integrationos/node-android-arm-eabi')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(
          join(__dirname, 'node.win32-x64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./node.win32-x64-msvc.node')
          } else {
            nativeBinding = require('@integrationos/node-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(
          join(__dirname, 'node.win32-ia32-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./node.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('@integrationos/node-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'node.win32-arm64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./node.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('@integrationos/node-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    localFileExisted = existsSync(join(__dirname, 'node.darwin-universal.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./node.darwin-universal.node')
      } else {
        nativeBinding = require('@integrationos/node-darwin-universal')
      }
      break
    } catch {}
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'node.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./node.darwin-x64.node')
          } else {
            nativeBinding = require('@integrationos/node-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'node.darwin-arm64.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./node.darwin-arm64.node')
          } else {
            nativeBinding = require('@integrationos/node-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'node.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./node.freebsd-x64.node')
      } else {
        nativeBinding = require('@integrationos/node-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'node.linux-x64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./node.linux-x64-musl.node')
            } else {
              nativeBinding = require('@integrationos/node-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'node.linux-x64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./node.linux-x64-gnu.node')
            } else {
              nativeBinding = require('@integrationos/node-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'node.linux-arm64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./node.linux-arm64-musl.node')
            } else {
              nativeBinding = require('@integrationos/node-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'node.linux-arm64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./node.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('@integrationos/node-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'node.linux-arm-musleabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./node.linux-arm-musleabihf.node')
            } else {
              nativeBinding = require('@integrationos/node-linux-arm-musleabihf')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'node.linux-arm-gnueabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./node.linux-arm-gnueabihf.node')
            } else {
              nativeBinding = require('@integrationos/node-linux-arm-gnueabihf')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'riscv64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'node.linux-riscv64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./node.linux-riscv64-musl.node')
            } else {
              nativeBinding = require('@integrationos/node-linux-riscv64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'node.linux-riscv64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./node.linux-riscv64-gnu.node')
            } else {
              nativeBinding = require('@integrationos/node-linux-riscv64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 's390x':
        localFileExisted = existsSync(
          join(__dirname, 'node.linux-s390x-gnu.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./node.linux-s390x-gnu.node')
          } else {
            nativeBinding = require('@integrationos/node-linux-s390x-gnu')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { UnifiedApi, IntegrationOS, Format, CustomerEligibility, Gender, ChannelAvailability, TargetSelection, MinimumRequirements, CustomerSelection, AccountType, Status, AllocationMethod, TargetType, SkuValidation, CreditType, Roles, TransactionMethod, AppliesTo, Country, GlobalTaxType, IdentityProvider, AccountEngagementLevel, ReviewApprovalStatus, StakeholderType, AccountStatus, CampaignStatus, CustomerType, MessageContentType, ReactionType, SupportTicketPriority, IssueLifecycleStatus, OrderStatus, DiscountType, FulfillmentStatus, ProductAvailabilityStatus, ExpenseApprovalStatus, FinancialChargeType, BillingStatus, InvoiceAdjustmentType, DataType, EmploymentType, TimeCycle, FinancialTrackingCategories, CampaignType, ContactAddressType, LeadLifecycleStatus, ParticipantEngagementStatus, ItemEntityStatus, Currency, CustomerEligibilityStatus, ItemCondition, TransactionType, EntityLifecycleStatus, JournalEntryStatus, DayOfWeek, AccountingAccountType, MessageReadStatus, MessageDeliveryStatus, SupportQueryType, ImageMimeType, OrderingCriteria, ChatType, ItemAvailabilityStatus, FinancialAccountStatus, TransactionStatus, CreditNoteStatus, PaymentTerm, EmploymentAndCandidateStatus, OrganizationalRole, CustomerStatus, SocialPlatform, ConversationStatus, CommunicationRole, LifecycleStatus, PaymentMethod, FinancialTransactionStatus, AuditOpinionType, InventoryStorageType, VisibilityScope, PaymentStatus, TransactionChannel, FinancialDisputeStatus, DeviceUsageType, FinancialTransactionType, AccessControlModel, ParticipantType, PriorityLevel, PrerequisiteRangeType, ContentVisibility, EntityCategory, GenderIdentity, CommunicationMethod, EmailCategoryType, UniversalIdentifierType, TaskStatus, UserStatus, PromotionType, MimeType } = nativeBinding

module.exports.UnifiedApi = UnifiedApi
module.exports.IntegrationOS = IntegrationOS
module.exports.Format = Format
module.exports.CustomerEligibility = CustomerEligibility
module.exports.Gender = Gender
module.exports.ChannelAvailability = ChannelAvailability
module.exports.TargetSelection = TargetSelection
module.exports.MinimumRequirements = MinimumRequirements
module.exports.CustomerSelection = CustomerSelection
module.exports.AccountType = AccountType
module.exports.Status = Status
module.exports.AllocationMethod = AllocationMethod
module.exports.TargetType = TargetType
module.exports.SkuValidation = SkuValidation
module.exports.CreditType = CreditType
module.exports.Roles = Roles
module.exports.TransactionMethod = TransactionMethod
module.exports.AppliesTo = AppliesTo
module.exports.Country = Country
module.exports.GlobalTaxType = GlobalTaxType
module.exports.IdentityProvider = IdentityProvider
module.exports.AccountEngagementLevel = AccountEngagementLevel
module.exports.ReviewApprovalStatus = ReviewApprovalStatus
module.exports.StakeholderType = StakeholderType
module.exports.AccountStatus = AccountStatus
module.exports.CampaignStatus = CampaignStatus
module.exports.CustomerType = CustomerType
module.exports.MessageContentType = MessageContentType
module.exports.ReactionType = ReactionType
module.exports.SupportTicketPriority = SupportTicketPriority
module.exports.IssueLifecycleStatus = IssueLifecycleStatus
module.exports.OrderStatus = OrderStatus
module.exports.DiscountType = DiscountType
module.exports.FulfillmentStatus = FulfillmentStatus
module.exports.ProductAvailabilityStatus = ProductAvailabilityStatus
module.exports.ExpenseApprovalStatus = ExpenseApprovalStatus
module.exports.FinancialChargeType = FinancialChargeType
module.exports.BillingStatus = BillingStatus
module.exports.InvoiceAdjustmentType = InvoiceAdjustmentType
module.exports.DataType = DataType
module.exports.EmploymentType = EmploymentType
module.exports.TimeCycle = TimeCycle
module.exports.FinancialTrackingCategories = FinancialTrackingCategories
module.exports.CampaignType = CampaignType
module.exports.ContactAddressType = ContactAddressType
module.exports.LeadLifecycleStatus = LeadLifecycleStatus
module.exports.ParticipantEngagementStatus = ParticipantEngagementStatus
module.exports.ItemEntityStatus = ItemEntityStatus
module.exports.Currency = Currency
module.exports.CustomerEligibilityStatus = CustomerEligibilityStatus
module.exports.ItemCondition = ItemCondition
module.exports.TransactionType = TransactionType
module.exports.EntityLifecycleStatus = EntityLifecycleStatus
module.exports.JournalEntryStatus = JournalEntryStatus
module.exports.DayOfWeek = DayOfWeek
module.exports.AccountingAccountType = AccountingAccountType
module.exports.MessageReadStatus = MessageReadStatus
module.exports.MessageDeliveryStatus = MessageDeliveryStatus
module.exports.SupportQueryType = SupportQueryType
module.exports.ImageMimeType = ImageMimeType
module.exports.OrderingCriteria = OrderingCriteria
module.exports.ChatType = ChatType
module.exports.ItemAvailabilityStatus = ItemAvailabilityStatus
module.exports.FinancialAccountStatus = FinancialAccountStatus
module.exports.TransactionStatus = TransactionStatus
module.exports.CreditNoteStatus = CreditNoteStatus
module.exports.PaymentTerm = PaymentTerm
module.exports.EmploymentAndCandidateStatus = EmploymentAndCandidateStatus
module.exports.OrganizationalRole = OrganizationalRole
module.exports.CustomerStatus = CustomerStatus
module.exports.SocialPlatform = SocialPlatform
module.exports.ConversationStatus = ConversationStatus
module.exports.CommunicationRole = CommunicationRole
module.exports.LifecycleStatus = LifecycleStatus
module.exports.PaymentMethod = PaymentMethod
module.exports.FinancialTransactionStatus = FinancialTransactionStatus
module.exports.AuditOpinionType = AuditOpinionType
module.exports.InventoryStorageType = InventoryStorageType
module.exports.VisibilityScope = VisibilityScope
module.exports.PaymentStatus = PaymentStatus
module.exports.TransactionChannel = TransactionChannel
module.exports.FinancialDisputeStatus = FinancialDisputeStatus
module.exports.DeviceUsageType = DeviceUsageType
module.exports.FinancialTransactionType = FinancialTransactionType
module.exports.AccessControlModel = AccessControlModel
module.exports.ParticipantType = ParticipantType
module.exports.PriorityLevel = PriorityLevel
module.exports.PrerequisiteRangeType = PrerequisiteRangeType
module.exports.ContentVisibility = ContentVisibility
module.exports.EntityCategory = EntityCategory
module.exports.GenderIdentity = GenderIdentity
module.exports.CommunicationMethod = CommunicationMethod
module.exports.EmailCategoryType = EmailCategoryType
module.exports.UniversalIdentifierType = UniversalIdentifierType
module.exports.TaskStatus = TaskStatus
module.exports.UserStatus = UserStatus
module.exports.PromotionType = PromotionType
module.exports.MimeType = MimeType
